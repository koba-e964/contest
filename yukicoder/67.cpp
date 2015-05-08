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

const int N = 200010;
int n;
ll l[N];
double k;

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> l[i];
  }
  cin >> k;
  double lo = l[0] / k, hi = 1e10;
  REP(loop_counter_0, 0, 100) {
    double mid = (lo + hi) / 2;
    ll sum = 0;
    REP(i, 0, n) {
      sum += floor(l[i] / mid);
    }
    if (sum >= k) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  printf("%.10f\n", lo);
}
