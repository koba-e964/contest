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


const int N = 100010;
ll c[N];
bool cap[N];
int tot2;
ll tot[N];
int main(void){
  int n, k;
  scanf("%d%d", &n, &k);
  REP(i, 0, n) {
    scanf("%lld", c + i);
  }
  REP(i, 0, k) {
    int t;
    scanf("%d", &t);
    cap[t - 1] = true;
  }
  ll sum = 0;
  REP(i, 0, n) {
    sum += c[i] * c[(i + 1) % n];
  }
  REP(i, 0, n) {
    tot2 += c[i];
  }
  for (int i = n - 1; i >= 0; i--) {
    tot[i] = tot[i + 1] + (cap[i] ? c[i] : 0);
  }
  REP(i, 0, n) {
    if (cap[i]) {
      sum += c[i] * (tot2 - c[i] - c[(i + 1) % n] - c[(i + n - 1) % n]);
    }
  }
  REP(i, 0, n - 1) {
    if (cap[i]) {
      sum -= c[i] * tot[i + 2];
      if (i == 0 && cap[n - 1]) {
	sum += c[0] * c[n - 1];
      }
    }
  }
  cout << sum << endl;
}
