#include "testlib.h"

using namespace std;

int main(int argc,char**argv) {
  registerValidation(argc,argv);
  int n = inf.readInt(1, 100000, "N");
  inf.readEoln();
  long long lim = 1e10;
  vector<long long> a = inf.readLongs(n, -lim, lim, "a");
  inf.readEoln();
  int q = inf.readInt(1, 100000, "Q");
  inf.readEoln();
  for (int _ = 0; _ < q; _++) {
    inf.readInt(1, 2, "k");
    inf.readSpace();
    int l = inf.readInt(1, n, "l");
    inf.readSpace();
    inf.readInt(l, n, "r");
    inf.readSpace();
    inf.readInt(-10000, 10000, "c");
    inf.readEoln();
  }
  inf.readEof();
}
