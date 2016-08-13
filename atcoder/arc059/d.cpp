#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
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
  int n = s.length();
  int x = -1, y = -1;
  REP(i, 0, n - 1) {
    if (s[i] == s[i + 1]) {
      x = i + 1;
      y = i + 2;
    }
  }
  REP(i, 0, n - 2) {
    int ch[26] = {0};
    REP(j, 0, 3) ch[s[i + j] - 'a']++;
    bool ok = 0;
    REP(j, 0, 26) {
      ok |= ch[j] >= 2;
    }
    if (ok) {
      x = i + 1;
      y = i + 3;
    }
  }
  cout << x << " " << y << endl;
}
