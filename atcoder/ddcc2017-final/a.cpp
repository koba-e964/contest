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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

bool in_circle(int x, int y, int n) {
  return (n - 2 * x) * (n - 2 * x) + (n - 2 * y) * (n - 2 * y) <= n * n;
}

int calc(int n) {
  int cnt = 0;
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (in_circle(i, j, n) &&
	  in_circle(i, j + 1, n) &&
	  in_circle(i + 1, j, n) &&
	  in_circle(i + 1, j + 1, n)) {
	cnt += 1;
      }
    }
  }
  return cnt;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int k;
  cin >> k;
  cout << calc(200 / k) << " " << calc(300 / k) << "\n";
}
