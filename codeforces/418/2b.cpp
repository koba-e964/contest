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

void output(const VI &a) {
  int n = a.size();
  REP(i, 0, n) {
    cout << a[i] << (i == n - 1 ? "\n" : " ");
  }
}

int dist(const VI &a, const VI &b) {
  int c = 0;
  int n = a.size();
  REP(i, 0, n) {
    c += a[i] != b[i];
  }
  return c;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  VI diff;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
    if (a[i] != b[i]) {
      diff.push_back(i);
    }
  }
  if (diff.size() == 1) {
    int idx = diff[0];
    set<int> rem;
    REP(i, 0, n) {
      rem.insert(i + 1);
    }
    REP(i, 0, n) {
      if (i != idx) {
	rem.erase(a[i]);
      }
    }
    a[idx] = *rem.begin();
    output(a);
    return 0;
  }
  assert (diff.size() == 2);
  int idx1 = diff[0];
  int idx2 = diff[1];
  set<int> rem;
  REP(i, 0, n) {
    rem.insert(i + 1);
  }
  REP(i, 0, n) {
    if (i != idx1 && i != idx2) {
      rem.erase(a[i]);
    }
  }
  set<int>::iterator it = rem.begin();
  int v1 = *it;
  it++;
  int v2 = *it;
  // Try 2 possibilities
  VI p(a);
  p[idx1] = v1;
  p[idx2] = v2;
  if (dist(a, p) == 1 && dist(b, p) == 1) {
    output(p);
    return 0;
  }
  p[idx1] = v2;
  p[idx2] = v1;
  output(p);
}
