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
  int n, k;
  cin >> n >> k;
  PI mi(1e8, -1);
  int lo = (n - 1) / k;
  int hi = lo + 1;
  int nhi = (n - 1) % k;
  int dist = nhi >= 2 ? 2 * hi : nhi == 1 ? hi + lo : 2 * lo;
  cout << dist << endl;
  int pos = 2;
  vector<PI> edges;
  REP(i, 0, nhi) {
    edges.push_back(PI(1, pos));
    pos++;
    REP(j, 1, hi) {
      edges.push_back(PI(pos - 1, pos));
      pos++;
    }
  }
  REP(i, 0, k - nhi) {
    edges.push_back(PI(1, pos));
    pos++;
    REP(j, 1, lo) {
      edges.push_back(PI(pos - 1, pos));
      pos++;
    }
  }
  REP(i, 0, edges.size()) {
    cout << edges[i].first << " " << edges[i].second << "\n";
  }
}
