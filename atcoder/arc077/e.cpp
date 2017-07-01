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

const int N = 123456 * 2;
ll acc[N], aux[N];

void add(int idx, ll v) {
  acc[0] += v;
  acc[idx + 1] -= v;
}

void add_spo(int idx, ll v) {
  aux[idx] += v;
}

int main(void){
  int n, m;
  cin >> n >> m;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll tot = 0;
  REP(i, 0, n - 1) {
    // a[i] (1 2 ... ??) a[i + 1]
    int diff = a[i + 1] - a[i];
    int dst = a[i + 1];
    if (diff < 0) {
      diff += m;
      dst += m;
    }
    tot += diff;
    // 1st-order diff table
    add(dst, 1);
    add(a[i] + 1, -1);
    add_spo(dst + 1, - diff + 1);
  }
  // acc
  REP(i, 1, N) {
    acc[i] += acc[i - 1];
  }
  REP(i, 0, N) {
    acc[i] += aux[i];
  }
  REP(i, 1, N) {
    acc[i] += acc[i - 1];
  }
  REP(i, 1, m + 1) {
    acc[i] += acc[i + m];
  }
  if (0) {
    REP(i, 0, m + 1) {
      cerr << " " << acc[i];
    }
    cerr << endl;
  }
  ll ma = 0;
  REP(i, 0, m + 1) {
    ma = max(ma, acc[i]);
  }
  cout << tot - ma << endl;
}
