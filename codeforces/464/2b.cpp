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
  ll n;
  int k;
  cin >> n >> k;
  VL a(k);
  REP(i, 0, k) {
    cin >> a[i];
  }
  pair<ll, int> ma(-1, -1);
  REP(i, 0, k) {
    ll v = n / a[i] * a[i];
    ma = max(ma, make_pair(v, i));
  }
  cout << ma.second + 1 << " " << ma.first / a[ma.second] << endl;
}
