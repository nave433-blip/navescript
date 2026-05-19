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
                Instruction::LOAD { reg, addr } => {
                    let val = builder.ins().iconst(types::I32, *addr as i64);
                    regs.insert(*reg, val);
                }
                Instruction::ADD { dest, src1, src2 } => {
                    if let (Some(&v1), Some(&v2)) = (regs.get(src1), regs.get(src2)) {
                        let res = builder.ins().iadd(v1, v2);
                        regs.insert(*dest, res);
                    }
                }
                Instruction::RET { reg } => {
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
