# NaveScript Homebrew Formula
class Nvs < Formula
  desc "NaveScript Language Ecosystem"
  homepage "https://navescript.org"
  url "https://github.com/navescript/nvs/archive/v0.1.0.tar.gz"
  
  def install
    system "make", "build"
    bin.install "bin/nvs"
  end
end
