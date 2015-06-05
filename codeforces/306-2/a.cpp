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
  int c = 0;
  REP (i, 0, s.length() - 1) {
    if (s[i] == 'A' && s[i + 1] == 'B') {
      c++;
    }
  }
  bool ok = false;
  REP (i, 0, s.length() - 1) {
    if (s[i] == 'B' && s[i + 1] == 'A') {
      int k = 0;
      k += i >= 1 && s[i - 1] == 'A';
      k += i < s.length() - 2 && s[i + 2] == 'B';
      if (c > k) {
	ok = true;
      }
    }
  }
  cout << (ok ? "YES" : "NO") << endl;
}
