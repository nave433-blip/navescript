class Navescript < Formula
  desc "The sovereign, self-hosted Navescript ecosystem"
  homepage "https://navescript.org"
  url "https://github.com/navescript/nvs/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_SHA256"

  def install
    system "make", "build"
    bin.install "bin/nvs"
    bin.install "bin/nvm"
  end
end
