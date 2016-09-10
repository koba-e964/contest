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

const int N = 11;
VI edges[N];
int main(void){
  int v,e;
  cin >> v >> e;
  REP(i, 0, e) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  REP(bits, 0, 1 << (2 * v)) {
    int col[N];
    REP(i, 0, v) {
      col[i] = (bits >> (2 * i)) & 3;
    }
    bool ok = 1;
    REP(i, 0, v) {
      REP(j, 0, edges[i].size()) {
	ok &= col[i] != col[edges[i][j]];
      }
    }
    if (ok) {
      REP(i, 0, v) {
	cout << col[i] + 1 << endl;
      }
      return 0;
    }
  }
  assert (0);
}
