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
  int u = 0;
  REP(i, 0, s.length()) {
    switch (u) {
    case 0:
      if (s[i] == 'a') {
	continue;
      }
      u = 1;
      // fall-through
    case 1:
      if (s[i] == 'a') {
	u = 2;
	continue;
      }
      s[i]--;
      break;
    case 2:
      break;
    }
  }
  if (u == 0) { // s = "aaaa..."
    s[s.length() - 1] = 'z';
  }
  cout << s << endl;
}
