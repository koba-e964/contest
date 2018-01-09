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

void succeed(void) {
  cout << "YES\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int a[3];
  REP(i, 0, 3) {
    cin >> a[i];
  }
  sort(a, a + 3);
  if (a[0] == 1) {
    succeed();
  }
  if (a[0] == 3 && a[2] == 3) {
    succeed();
  }
  if (a[0] == 2 && a[1] == 2) {
    succeed();
  }
  if (a[0] == 2 && a[1] == 4 && a[2] == 4) {
    succeed();
  }
  cout << "NO\n";
}
