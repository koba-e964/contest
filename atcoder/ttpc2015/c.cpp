#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  string s;
  cin >> s;
  reverse(s.begin(), s.end());
  string t;
  int p = 0;
  while (p < s.length()) {
    if (s.substr(p, 9) != "amayakooo") {
      t += s[p];
      p++;
      continue;
    }
    t += "amayak";
    p += 9;
    int w = 3;
    while (p < s.length() && s[p] == 'o') {
      p++;
      w++;
    }
    char const *tbl[3] = {"oO", "o", "O"};
    t = t + tbl[w % 3];
  }
  reverse(t.begin(), t.end());
  cout << t << endl;
}
