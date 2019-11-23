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

const int N = 10000000;
int acc[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll p, q;
  cin >> p >> q;
  if (p == 0) {
    cout << 0 << endl;
    return 0;
  }
  for (int a = 1; a * a < N; a++) {
    for (int b = 1; b * b + a * a < N; b++) {
      acc[a * a + b * b] += 4;
    }
  }
  REP(i, 0, N - 1) acc[i + 1] += acc[i];
  REP(i, 1, N) {
    // acc[i] / i >= p / q ?
    if (acc[i] * q >= i * p) {
      cout << (i * p + q - 1) / q << endl;
      return 0;
    }
  }
}
