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

#define IS2(x) ((x) == '2' || (x) == '?')
#define IS5(x) ((x) == '5' || (x) == '?')

int solve(VI &v) {
  int cur = 0;
  int ma = 0;
  REP(i, 0, v.size()) {
    if (v[i] == 1) {
      cur++;
    } else {
      cur = 0;
    }
    ma = max(ma, cur);
  }
  return ma;
}

int main(void){
  string t;
  cin >> t;
  int n = t.length();
  VI odd(n, 0), even(n, 0);
  REP(i, 0, n) {
    if (i % 2 == 0 && i + 1 < n) {
      even[i / 2] = IS2(t[i]) && IS5(t[i + 1]);
    }
    if (i % 2 == 1) {
      odd[i / 2] = IS2(t[i]) && IS5(t[i + 1]); 
    }
  }
  int ma = max(solve(odd), solve(even));
  cout << ma * 2 << endl;
}
