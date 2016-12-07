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
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    a[i]--, b[i]--;
  }
  VI geg(2 * n);
  REP(i, 0, n) {
    geg[a[i]] = b[i];
    geg[b[i]] = a[i];
  }
  VI visited(2 * n);
  VI sol(2 * n);
  REP(i, 0, 2 * n) {
    if (visited[i]) {
      continue;
    }
    VI seq;
    seq.push_back(i);
    int v = geg[i];
    visited[i] = 1;
    visited[v] = 1;
    seq.push_back(v);
    v ^= 1;
    while (v != i) {
      visited[v] = 1;
      seq.push_back(v);
      v = geg[v];
      visited[v] = 1;
      seq.push_back(v);
      v ^= 1;
    }
    assert (seq.size() % 2 == 0);
    REP(i, 0, seq.size()) {
      sol[seq[i]] = 1 + i % 2;
    }
  }
  REP(i, 0, n) {
    cout << sol[a[i]] << " " << sol[b[i]] << "\n";
  }
}
