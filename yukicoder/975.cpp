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
  int x, n, m;
  cin >> x >> n >> m;
  set<int> a, b;
  REP(i, 0, n) {
    int v; cin >> v;
    a.insert(v);
  }
  REP(i, 0, m) {
    int v; cin >> v;
    b.insert(v);
  }
  int ty = a.count(x) * 2 + b.count(x);
  if (ty == 0) {
    cout << -1 << endl;
  } else if (ty == 1) {
    cout << "MaxValu" << endl;
  } else if (ty == 2) {
    cout << "MrMax" << endl;
  } else {
    cout << "MrMaxValu" << endl;
  }
}
