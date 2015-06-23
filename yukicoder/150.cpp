#include <iostream>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int solve(string s) {
  int l = s.length();
  int ma = 11;
  REP(i, 0, l - 10) {
    REP(j, i + 4, l - 6) {
      int c = 0;
      REP(k, 0, 4) {
	c += s[i + k] != "good"[k];
      }
      REP(k, 0, 7) {
	c += s[j + k] != "problem"[k];
      }
      ma = min(ma, c);
    }
  }
  return ma;
}

int main(void){
  int t;
  cin >> t;
  REP(loop_var, 0, t) {
    string s;
    cin >> s;
    cout << solve(s) << endl;
  }
}
