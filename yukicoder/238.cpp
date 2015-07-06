#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

bool pal(const string &s) {
  string t(s);
  reverse(t.begin(), t.end());
  return s == t;
}

string solve(const string &s) {
  int n = s.length();
  if (n % 2 == 0 && pal(s)) {
    return s.substr(0, n / 2) + "v" + s.substr(n / 2);
  }
  int v = 0;
  while (v < n / 2) {
    if (s[v] != s[n - 1 - v]) {
      break;
    }
    v++;
  }
  string u1 = s.substr(0, v) + s[n - 1 - v] + s.substr(v);
  string u2 = s.substr(0, n - v) + s[v] + s.substr(n - v);
  if(pal(u1)) return u1;
  if(pal(u2)) return u2;
  return "NA";
}

int main(void){
  string s;
  cin >> s;
  cout << solve(s) << endl;
}
