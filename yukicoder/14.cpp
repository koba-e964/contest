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

int n;
const int N = 10010;

int a[N];

const int K = 1000;
int memo[K][K];

int gcd(int x, int y) {
  if (y == 0) return x;
  if (x < K && y < K && memo[x][y] > 0) {
    return memo[x][y];
  }
  int r = x % y;
  int ans = gcd(y, r);
  if (x < K && y < K) memo[x][y] = ans;
  return ans;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n - 1) {
    int mini = -1;
    int minl = 0;
    REP(k, i + 1, n) {
      int l = a[k] / gcd(a[i], a[k]);
      if (mini == -1 || minl > l ||
          (minl == l && a[mini] > a[k])) {
        mini = k;
        minl = l;
      }
    }
    if (i + 1 != mini) swap(a[i + 1], a[mini]);
  }
  REP(i, 0, n) {
    cout << a[i] << (i == n - 1 ? "\n" : " ");
  }
}
