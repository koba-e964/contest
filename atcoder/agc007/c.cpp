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

const int N = 200100;
double t[N], u[N];

int main(void){
  int n;
  double d0, x;
  cin >> n >> d0 >> x;
  t[1] = 0.5;
  u[1] = 0;
  REP(i, 1, N - 1) {
    t[i + 1] = t[i] * (i + 2) / (i + 1) + 0.5;
    u[i + 1] = u[i] * (i + 3) / (i + 1) + t[i] * 3.0 / (2 * i + 2) + i / 4.0;
  }
  printf("%.9f\n", (2 * d0 + 1 * x) * t[n] + 4 * x * u[n]);
}
