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

const ll inf = 1e15;


int main(void){
  int n;
  cin >> n;
  VL a(3 * n);
  REP(i, 0, 3 * n) {
    cin >> a[i];
  }
  VL left(3 * n, -inf);
  {
    priority_queue<ll, vector<ll>, greater<ll> > que;
    ll sum = 0;
    REP(i, 0, 3 * n) {
      sum += a[i];
      que.push(a[i]);
      if (i >= n) {
	ll t = que.top(); que.pop();
	sum -= t;
      }
      if (i >= n - 1) {
	left[i] = sum;
      }
    }
  }
  VL right(3 * n, inf);
  {
    priority_queue<ll, vector<ll>, less<ll> > que;
    ll sum = 0;
    for (int i = 3 * n - 1; i >= 0; --i) {
      sum += a[i];
      que.push(a[i]);
      if (i <= 2 * n - 1) {
	ll t = que.top(); que.pop();
	sum -= t;
      }
      if (i <= 2 * n) {
        right[i] = sum;
      }
    }
  }
  if (0) {
    REP(i, 0, 3 * n) {
      cerr << left[i] << " ";
    }
    cerr << endl;
    REP(i, 0, 3 * n) {
      cerr << right[i] << " ";
    }
    cerr << endl;
  }
  ll ma = -inf;
  REP(i, 0, 3 * n - 1) {
    ma = max(left[i] - right[i + 1], ma);
  }
  cout << ma << endl;
}
