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

const int N = 101;
map<int, bitset<N> > edge[N];
int d[1010];
int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  REP(i, 0, m) {
    int a,b,c;
    cin >> a >> b >> c;
    a--, b--;
    if (edge[a].count(c) == 0) {
      edge[a][c] = bitset<N>();
    }
    if (edge[b].count(c) == 0) {
      edge[b][c] = bitset<N>();
    }
    edge[a][c].set(b);
    edge[b][c].set(a);
  }
  REP(i, 0, k) {
    cin >> d[i];
  }
  bitset<N> bs;
  bs.flip();
  for (int i = 0; i < k; ++i) {
    bitset<N> sub;
    REP(j, 0, n) {
      if (bs[j]) {
	sub |= edge[j][d[i]];
      }
    }
    bs = sub;
  }
  cout << bs.count() << endl;
  int c = 0;
  REP (i, 0, bs.size()) {
    if (bs[i]) {
      if (c) {
	cout << " ";
      }
      cout << i + 1;
      c = 1;
    }
  }
  cout << endl;
}
