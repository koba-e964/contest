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
  int l = 0, r = 0, u = 0, d = 0;
  REP(i, 0, s.length()) {
    switch(s[i]) {
    case 'L':
      l++;
      break;
    case 'R':
      r++;
      break;
    case 'U':
      u++;
      break;
    case 'D':
      d++;
      break;
    }
  }
  if (s.length() % 2 == 1) {
    cout << -1 << endl;
    return 0;
  }
  int mi = 100000000;
  int n = s.length() / 2;
  REP(i, 0, n + 1) {
    mi = min(mi, abs(l - i) + abs(r - i) + abs(u - n + i) + abs(d - n + i));
  }
  cout << mi / 2 << endl;
}
