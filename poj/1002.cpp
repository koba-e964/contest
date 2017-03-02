#include <algorithm>
#include <cassert>
#include <cstdio>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int normalize(const string &s) {
  int ret = 0;
  REP(i, 0, s.length()) {
    char c = s[i];
    if (c == '-') { continue; }
    const int tbl[26] = {0, 0, 0, 1, 1, 1, 2, 2, 2,
		   3, 3, 3, 4, 4, 4, 5, -1, 5, 5,
		   6, 6, 6, 7, 7, 7, -1};
    if (c > '9') {
      int tmp = tbl[c - 'A'];
      assert (tmp >= 0);
      c = tmp + '2';
    }
    ret *= 10;
    ret += c - '0';
  }
  return ret;
}
char buf[70];

const int W = 10000000;
int reg[W];

int main(void){
  int n;
  scanf("%d", &n);
  REP(i, 0, n) {
    string s;
    scanf("%69s", buf);
    s = buf;
    int nf = normalize(s);
    reg[nf] += 1;
  }
  int cnt = 0;
  REP(i, 0, W) {
    if (reg[i] >= 2) {
      cnt += 1;
      printf("%03d-%04d %d\n", i / 10000,
	     i % 10000,
	     reg[i]);
    }
  }
  if (cnt == 0) {
    puts("No duplicates.");
  }
}
