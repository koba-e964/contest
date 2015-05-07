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
int a[5010];

void swap(int i, int j) {
  int t = a[i];
  a[i] = a[j];
  a[j] = t;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  int k = 0;
  int r = 0;
  while(k <= 14) {
    int r2 = r;
    while(r2 < n && (a[r2] & (1 << k)) == 0) {
      ++r2;
    }
    if (r2 == n) {
      ++k;
      continue;
    }
    swap(r, r2);
    REP(i, r + 1, n) {
      if ((a[i] & (1 << k)) != 0) {
	a[i] ^= a[r];
      }
    }
    ++r;
    ++k;
  }
  cout << (1 << r) << endl;
}
