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

const int DEBUG = 0;

bool check(const VI& v) {
  vector<PI> cont;
  int cur = v[0];
  int cnt = 1;
  REP(i, 1, v.size() + 1) {
    if (i >= v.size() || cur != v[i]) {
      cont.push_back(PI(cur, cnt));
      cnt = 0;
    }
    if (i < v.size()) {
      cur = v[i];
      cnt++;
    }
  }
  vector<PI> cont2;
  cnt = 0;
  REP(i, 0, cont.size() + 1) {
    if (i >= cont.size() || cont[i].second != 1) {
      if (cnt >= 1) {
	cont2.push_back(PI(-1, cnt));
	cnt = 0;
      }
    }
    if (i < cont.size()) {
      PI t = cont[i];
      if (t.second == 1) {
	cnt++;
      } else {
	cont2.push_back(t);
      }
    }
  }
  if (DEBUG) {
    cerr << "cont:" << endl;
    REP(i, 0, cont.size()) {
      cerr << (cont[i].first ? "b" : "w") << cont[i].second << " ";
    }
    cerr << endl << "cont2:" << endl;
    REP(i, 0, cont2.size()) {
      cerr << (cont2[i].first == -1 ? "m" : cont2[i].first ? "b" : "w")
	   << cont2[i].second << " ";
    }
    cerr << endl;
  }
  if (cont2.size() == 1 && cont2[0].first == -1) { // (w)bwbwbwbw...
    return v[0];
  }
  int c2s = cont2.size();
  REP(i, 0, c2s) {
    if (cont2[i].first >= 0) {
      continue;
    }
    int stripe = cont2[i].second;
    if (i >= 1 && i < c2s - 1) {
      cont2[i - 1].second += stripe / 2;
      cont2[i].second -= stripe / 2;
      cont2[i + 1].second += stripe - stripe / 2;
      cont2[i].second -= stripe - stripe / 2;
    }
    if (i == 0) {
      cont2[1].second += stripe;
      cont2[0].second -= stripe;
    }
    if (i == c2s - 1) {
      cont2[c2s - 2].second += stripe;
      cont2[c2s - 1].second -= stripe;
    }
  }
  if (DEBUG) {
    cerr << "cont2-mod:" << endl;
    REP(i, 0, cont2.size()) {
      cerr << (cont2[i].first == -1 ? "m" : cont2[i].first ? "b" : "w")
	   << cont2[i].second << " ";
    }
    cerr << endl;
  }
  int rem = v.size() / 2;
  REP(i, 0, cont2.size()) {
    if (rem < cont2[i].second) {
      assert (cont2[i].first >= 0);
      return cont2[i].first;
    }
    rem -= cont2[i].second;
  }
  assert (0);
}

int main(void){
  int n;
  cin >> n;
  VI a(2 * n - 1);
  REP(i, 0, 2 * n - 1) {
    cin >> a[i];
  }
  int lo = 1, hi = 2 * n - 1;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    VI monochrome(2 * n - 1);
    REP(i, 0, 2 * n - 1) {
      monochrome[i] = a[i] >= mid;
    }
    if (DEBUG) {
      cerr << "mid = " << mid << endl;
    }
    if (check(monochrome)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  cout << lo << endl;
}
