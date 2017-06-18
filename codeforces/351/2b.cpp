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
  int n, m;
  cin >> n >> m;
  int lo = 1;
  int hi = n - 1;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    if (u > v) {
      swap(u, v);
    }
    lo = max(lo, u);
    hi = min(hi, v - 1);
  }
  cout << (lo <= hi ? hi - lo + 1 : 0) << endl;
}
