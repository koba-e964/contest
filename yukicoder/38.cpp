#include <iostream>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;



int main(void){
  int kr,kb, ma = 0;
  cin >> kr >> kb;
  string s;
  cin >> s;
  int mask = 0;
  int white = 0;
  REP(i, 0, s.length()) {
    mask |= s[i] == 'W' ? 0 : (1 << i);
    white |= s[i] != 'W' ? 0 : (1 << i);    
  }
  for (int bits = mask; bits >= 1 << ma; --bits) {
    bits &= mask;
    if (ma >= __builtin_popcount(bits | white)) {
      continue;
    }
    string t;
    REP (i, 0, s.length()) {
      if ((bits | white) & (1 << i)) {
	t += s[i];
      }
    }
    bool ok = true;
    REP (i, 0, t.length() - kr) {
      if (t[i] == 'R' && t[i + kr] == 'R') {
	ok = false;
      }
    }
    REP (i, 0, t.length() - kb) {
      if (t[i] == 'B' && t[i + kb] == 'B') {
	ok = false;
      }
    }
    if (ok) {
      ma = max(ma, (int) t.length());
    }
  }
  cout << ma << endl;
}
