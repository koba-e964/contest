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

const int N = 100100;
int a[N];
int cnt[N];

const ll mod = 1e9 + 7;
ll p2[2 * N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
    if (a[i] >= n) {
      cout << 0 << endl;
      return 0;
    }
    cnt[a[i]]++;
  }
  if (a[0] != 0 || cnt[0] != 1) {
    cout << 0 << endl;
    return 0;
  }
  ll tmp = 1;
  REP(i, 0, 2 * N) {
    p2[i] = tmp;
    tmp = 2 * tmp % mod;
  }
  ll sum = 1;
  REP(i, 1, N) {
    REP(j, 0, cnt[i]) {
      sum *= (p2[cnt[i - 1]] + mod - 1) % mod;
      sum %= mod;
      sum *= p2[j];
      sum %= mod;
    }
  }
  cout << sum << endl;
}
