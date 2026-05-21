use cranelift::prelude::*;
use cranelift_module::{Module, Linkage};
use cranelift_object::{ObjectBuilder, ObjectModule};
use anyhow::Result;
use cranelift::codegen::Context as CraneliftContext;
use crate::ir::{NSIr, Instruction};

pub struct CraneliftBackend {
    builder_context: FunctionBuilderContext,
    ctx: CraneliftContext,
    module: ObjectModule,
}

impl CraneliftBackend {
    pub fn new() -> Result<Self> {
        let flag_builder = settings::builder();
        let isa_builder = cranelift_native::builder()
            .map_err(|e| anyhow::anyhow!(e))?;
        let isa = isa_builder.finish(settings::Flags::new(flag_builder))?;
            
        let builder = ObjectBuilder::new(isa, "navescript", cranelift_module::default_libcall_names())?;
        let module = ObjectModule::new(builder);

        Ok(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: CraneliftContext::new(),
            module,
        })
    }

    pub fn compile_ir(&mut self, name: &str, ir: &NSIr) -> Result<()> {
        let mut func = self.ctx.func.clone();
        func.signature.returns.push(AbiParam::new(types::I32));

        let mut builder = FunctionBuilder::new(&mut func, &mut self.builder_context);
        let entry = builder.create_block();
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let mut regs: std::collections::HashMap<u8, Value> = std::collections::HashMap::new();

        let mut terminated = false;
        for instr in &ir.body {
            match instr {
                Instruction::Load { reg, addr } => {
                    let val = builder.ins().iconst(types::I32, *addr as i64);
                    regs.insert(*reg, val);
                }
                Instruction::Add { dest, src1, src2 } => {
                    if let (Some(&v1), Some(&v2)) = (regs.get(src1), regs.get(src2)) {
                        let res = builder.ins().iadd(v1, v2);
                        regs.insert(*dest, res);
                    }
                }
                Instruction::Ret { reg } => {
                    if let Some(&v) = regs.get(reg) {
                        builder.ins().return_(&[v]);
                        terminated = true;
                        break;
                    }
                }
                Instruction::Log(msg) => {
                    println!("📝 Cranelift [Compile-time Log]: {}", msg);
                }
                _ => {}
            }
        }

        if !terminated {
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().return_(&[zero]);
        }

        builder.finalize();
        self.ctx.func = func;

        let signature = self.ctx.func.signature.clone();
        let id = self.module.declare_function(name, Linkage::Export, &signature)?;
        self.module.define_function(id, &mut self.ctx)?;

        println!("✅ Cranelift: Compiled function '{}' from IR", name);
        Ok(())
    }

    pub fn finish(self) -> Result<Vec<u8>> {
        let obj = self.module.finish();
        Ok(obj.emit()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cranelift_backend_basic() {
        let mut backend = CraneliftBackend::new().unwrap();
        let ir = NSIr {
            module_name: "test_cranelift".to_string(),
            world: "cli".to_string(),
            imports: vec![],
            requirements: vec![],
            resources: vec![],
            body: vec![
                Instruction::Load { reg: 0, addr: 10 },
                Instruction::Load { reg: 1, addr: 20 },
                Instruction::Add { dest: 2, src1: 0, src2: 1 },
                Instruction::Ret { reg: 2 },
            ],
        };

        // Should compile correctly without panicking
        backend.compile_ir("main", &ir).unwrap();
        let binary = backend.finish().unwrap();
        
        // Output should have some bytes representing the mach-o/elf/etc object
        assert!(binary.len() > 0);
    }
}
