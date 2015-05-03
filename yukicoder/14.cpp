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

int n;
const int N = 10010;

int a[N];

int b[N];
int u[N];

int lcm(int x, int y) {
  return x / __gcd(x, y) * y;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
    u[i] = 0;
  }
  int s = 0;
  REP(i, 0, n) {
    b[i] = a[s];
    u[s] = 1;
    int mini = -1;
    int minl = 0;
    REP(k, 0, n) {
      if (u[k]) {
        continue;
      }
      int l = lcm(a[s], a[k]);
      if (mini == -1 || minl > l ||
          (minl == l && a[mini] > a[k])) {
        mini = k;
        minl = l;
      }
    }
    s = mini;
  }
  REP(i, 0, n) {
    cout << b[i] << (i == n - 1 ? "\n" : " ");
  }
}
