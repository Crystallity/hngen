# hngen

'''
hngen (Head Nonce Generator) is a CLI tool to find a nonce which hashes to a value beginning with a specified character string

Usage: hngen [OPTIONS] --head <HEAD>

Options:
  -s, --head <HEAD>        String to search for
  -t, --threads <THREADS>  Number of threads to dispatch [default: 16]
      --verbose            Instruct threads to print all hashes
  -h, --help               Print help
  -V, --version            Print version
'''
