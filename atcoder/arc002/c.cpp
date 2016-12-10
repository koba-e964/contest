#include <iostream>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;


int solve(const string &s, const string &c, const string &d) {
  int cnt = 0;
  int n = s.length();
  int pos = 0;
  while (pos < n) {
    if (pos < n - 1 &&
	(s.substr(pos, 2) == c || s.substr(pos, 2) == d)) {
      pos += 2;
    } else {
      pos++;
    }
    cnt++;
  }
  return cnt;
}

int main(void){
  int n;
  cin >> n;
  string s;
  cin >> s;
  string pat = "ABXY";
  int mi = n;
  REP(i, 0, 4) {
    REP(j, 0, 4) {
      string c = "  ";
      c[0] = pat[i];
      c[1] = pat[j];
      REP(k, 0, 4) {
	REP(l, 0, 4) {
	  string d = "  ";
	  d[0] = pat[k];
	  d[1] = pat[l];
	  mi = min(mi, solve(s, c, d));
	}
      }
    }
  }
  cout << mi << endl;
}
