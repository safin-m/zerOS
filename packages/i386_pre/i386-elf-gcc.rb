require 'formula'

class I386ElfGcc < Formula
  homepage 'http://gcc.gnu.org'
  url 'http://ftpmirror.gnu.org/gcc/gcc-4.9.3/gcc-4.9.3.tar.bz2'
  mirror 'http://ftp.gnu.org/gnu/gcc/gcc-4.9.3/gcc-4.9.3.tar.bz2'
  sha256 '1934f3f68fe477299fe929e527c62d800f8b1b01ca74cc218f90459715ace296'
  revision 1

  depends_on 'gmp'
  depends_on 'libmpc'
  depends_on 'mpfr'
  depends_on 'i386-elf-binutils'

  def install
    binutils = Formula.factory 'i386-elf-binutils'

    ENV['CC'] = '/usr/local/bin/gcc-4.9'
    ENV['CXX'] = '/usr/local/bin/g++-4.9'
    ENV['CPP'] = '/usr/local/bin/cpp-4.9'
    ENV['LD'] = '/usr/local/bin/gcc-4.9'
    ENV['PATH'] += ":#{binutils.prefix/"bin"}"

    mkdir 'build' do
      system '../configure', '--disable-nls', '--target=i386-elf', '--disable-werror',
                             "--prefix=#{prefix}",
                             "--enable-languages=c",
                             "--without-headers",
                             "--with-gmp=#{Formula["gmp"].opt_prefix}",
                             "--with-mpfr=#{Formula["mpfr"].opt_prefix}",
                             "--with-mpc=#{Formula["libmpc"].opt_prefix}"
      system 'make all-gcc'
      system 'make install-gcc'
      FileUtils.ln_sf binutils.prefix/"i386-elf", prefix/"i386-elf"
      system 'make all-target-libgcc'
      system 'make install-target-libgcc'
      FileUtils.rm_rf share/"man"/"man7"
    end
  end
end
