#include <algorithm>
#include <string>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int solve(const string &s) {
  int pos = 0;
  int n = s.length();
  int cnt = 0;
  while (pos < n) {
    if (s.substr(pos, min(5, n - pos)) == "kyoto") {
      pos += 5;
      cnt += 1;
      continue;
    }
    if (s.substr(pos, min(5, n - pos)) == "tokyo") {
      pos += 5;
      cnt += 1;
      continue;
    }
    pos += 1;
  }
  return cnt;
}

int main(void){
  int t;
  cin >> t;
  REP(i, 0, t) {
    string s;
    cin >> s;
    cout << solve(s) << endl;
  }
}
