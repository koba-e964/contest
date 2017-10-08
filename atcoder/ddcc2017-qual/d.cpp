#include <algorithm>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

ll calc(int t4, int t3, int t2sn, ll a, ll b) {
  ll tot = 0;
  t2sn += t3;
  t3 = 0;
  tot += a * t2sn;
  t2sn = 0;
  tot += t4 * (max(a, b) + a + b);
  return tot;
}

int main(void) {
  int h, w;
  cin >> h >> w;
  ll a, b;
  cin >> a >> b;
  vector<string> s(h);
  REP(i, 0, h) {
    cin >> s[i];
  }
  int t4 = 0;
  int t2we = 0;
  int t2sn = 0;
  int t3 = 0;
  bool init_sn = true;
  bool init_we = true;
  REP(i, 0, h / 2) {
    REP(j, 0, w / 2) {
      int bits = 0;
      bits |= s[i][j] == 'S' ? 1 : 0;
      bits |= s[i][w - 1 - j] == 'S' ? 2 : 0;
      bits |= s[h - 1 - i][j] == 'S' ? 4 : 0;
      bits |= s[h - 1 - i][w - 1 - j] == 'S' ? 8 : 0;
      int pop = __builtin_popcount(bits);
      if (pop == 3) {
	t3 += 1;
	init_sn = false;
	init_we = false;
      }
      if (pop == 4) {
	t4 += 1;
      }
      if (pop == 2) {
	if (bits == 9 || bits == 6) {
	  init_sn = false;
	  init_we = false;
	} else if (bits == 0 || bits == 3 || bits == 12 || bits == 15) { // yoko
	  t2we += 1;
	  init_sn = false;
	} else { // tate
	  t2sn += 1;
	  init_we = false;
	}
      }
      if (pop <= 1) {
	if (pop == 1) {
	  init_sn = false;
	  init_we = false;
	}
      }
    }
  }
  ll ma = 0;
  ma = max(ma, calc(t4, t3, t2sn, a, b));
  ma = max(ma, calc(t4, t3, t2we, b, a));
  ma += a + b;
  if (init_sn) {
    ma -= a;
  }
  if (init_we) {
    ma -= b;
  }
  cout << ma << endl;
}
