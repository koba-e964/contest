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


int n, k;
const int N = 200010;
double p[N], acc[N];
int main(void) {
  cin >> n >> k;
  REP(i, 0, n) {
    int v;
    cin >> v;
    p[i] = (v + 1) / 2.0;
    acc[i + 1] = acc[i] + p[i];
  }
  double ma = 0.0;
  REP(i, 0, n - k + 1) {
    ma = max(ma, acc[i + k] - acc[i]);
  }
  printf("%.15f\n", ma);
}
