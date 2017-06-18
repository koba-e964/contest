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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  string s;
  cin >> s;
  int n = s.length();
  REP(i, 0, n) {
    // ranges
    vector<VI> ranges(26, VI(n - i, 0));
    REP(j, 0, n) {
      int c = s[j] - 'a';
      REP(k, max(j - i, 0), min(j, n - 1 - i) + 1) {
	ranges[c][k] = 1;
      }
    }
    // exists some letter
    REP(c, 0, 26) {
      bool all = true;
      REP(j, 0, n - i) {
	if (ranges[c][j] == 0) {
	  all = false;
	  break;
	}
      }
      if (all) {
	cout << i << endl;
	return 0;
      }
    }
  }
  assert (0);
}
