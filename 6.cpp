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

const int DEBUG = 0;

const int N = 200100;
int k, n;
int a[N];
int bucket[9];

int main(void){
  cin >> k >> n;
  REP(i, 0, N) {
    a[i] = 1;
  }
  a[0] = a[1] = 0;
  int c = 2;
  while (c <= n) {
    if (a[c] == 0) {
      c++;
      continue;
    }
    REP(i, 2, (N + 1) / c) {
      a[i * c] = 0;
    }
    c++;
  }
  int m = 0;
  int mlen = 0;
  int i = k, j = k;
  int cnt = 0;
  REP(q, 0, 9) {
    bucket[q] = 0;
  }
  while(i <= n) {
    if(a[i] == 0) {
      i++;
      j = max(i, j);
      continue;
    }
    for (;j <= n; ++j) {
      if (a[j] == 0) {
	continue;
      }
      if (bucket[j % 9]) {
	break;
      }
      cnt++;
      bucket[j % 9] = 1;
    }
    if (mlen <= cnt) {
      if (DEBUG) {
	cout << "i=" << i << ", j=" << j << ", cnt=" << cnt << endl;
      }
      m = i;
      mlen = cnt;
    }
    bucket[i % 9] = 0;
    i++;
    cnt--;
  }
  cout << m << endl;
}
