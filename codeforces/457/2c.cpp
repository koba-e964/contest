#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

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
  int p = 100003;
  cout << p << " " << p << endl;
  vector<pair<PI, int> > edges;
  REP(i, 0, n - 1) {
    edges.push_back(make_pair(PI(i, i + 1), i == 0 ? p - n + 2 : 1));
  }
  int c = 0;
  int d = 2;
  REP(conspectu, n - 1, m) {
    edges.push_back(make_pair(PI(c, d), 1e9));
    d += 1;
    if (d >= n) {
      c += 1;
      d = c + 2;
    }
  }
  assert ((int)edges.size() == m);
  REP(i, 0, m) {
    pair<PI, int> e = edges[i];
    cout << e.first.first + 1 << " " << e.first.second + 1 << " " << e.second << "\n";
  }
}
