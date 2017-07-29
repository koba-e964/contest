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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<set<int> > edges(n);
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].insert(b);
    edges[b].insert(a);
  }
  bool possum = false;
  REP(i, 1, n - 1) {
    if (edges[i].count(0) && edges[i].count(n - 1)) {
      possum = true;
      break;
    }
  }
  cout << (possum ? "POSSIBLE" : "IMPOSSIBLE") << "\n";
}
