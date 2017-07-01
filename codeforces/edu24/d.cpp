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
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  int aa;
  cin >> n >> aa;
  VI c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  map<int, int> freq;
  int target = 0;
  set<int> forbidden;
  set<int> app;
  REP(i, 0, n) {
    if (aa == c[i]) {
      target += 1;
      continue;
    }
    app.insert(c[i]);
    if (freq[c[i]] < target) {
      forbidden.insert(c[i]);
    }
    freq[c[i]] += 1;
  }
  set<int> diff;
  for(set<int>::iterator it = app.begin(); it != app.end(); ++it) {
    if (freq[*it] >= target && not forbidden.count(*it)) {
      diff.insert(*it);
    }
  }
  if (diff.size() == 0) {
    cout << -1 << "\n";
  } else {
    cout << *diff.begin() << "\n";
  }
}
