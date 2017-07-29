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

const int N = 50;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll k;
  cin >> k;
  cout << N << "\n";
  int r = (50 - k % 50) % 50;
  ll q = (k + r) / 50;
  VL a(N);
  REP(i, 0, r) {
    a[i] = r - 2 + q;
  }
  REP(i, r, N) {
    a[i] = N - 1 + r + q;
  }
  REP(i, 0, N) {
    cout << a[i] << (i == N - 1 ? "\n" : " ");
  }
}
