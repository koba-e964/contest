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
  VI a(k);
  int rem = 2 * n;
  REP(i, 0, k) {
    cin >> a[i];
    rem -= a[i] / 4;
    a[i] %= 4;
  }
  VI pool(4);
  REP(i, 0, k) {
    if (a[i] != 0) {
      pool[a[i]] += 1;
    }
  }
  int two = pool[2];
  int addup = pool[1] + pool[2] + 2 * pool[3];
  if (pool[1] + 2 * n < two) {
    int diff = two - pool[1] - 2 * n;
    addup += diff / 3 + (diff % 3);
  }
  if (2 * rem >= addup) {
    cout << "YES\n";
  } else {
    cout << "NO\n";
  }
}
