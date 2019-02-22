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

const int DEBUG = 0;


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

int n;

void ans_swap(VI &ans, int &pos, int j) {
  if (ans.size() > 100000) {
    assert (0);
  }
  while (pos != j + 1) {
    pos = (pos + 1) % n;
    ans.push_back(1);
  }
  ans.push_back(n - 1);
}

void shift(VI &ans, int &pos, int dest) {
  while (pos != dest) {
    pos = (pos + 1) % n;
    ans.push_back(1);
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
  }
  VI cp(p);
  VI ans;
  int pos = 0;
  REP(i, 0, n - 1) {
    REP(j, 0, n - i - 1) {
      if (p[j] > p[j + 1]) {
	swap(p[j], p[j + 1]);
	ans_swap(ans, pos, j);
      }
    }
  }
  shift(ans, pos, 0);
  cout << ans.size() << "\n";
  REP(i, 0, ans.size()) {
    cout << ans[i] << "\n";
  }
  if (DEBUG) {
    REP(i, 0, ans.size()) {
      int k = ans[i];
      for(int i=k;i<n;i++)
	swap(cp[i],cp[i-k]);
    }
    REP(i, 0, n) {
      cerr << "result: ";
      for(auto cpp:cp)cerr << " " << cpp;
      cerr << endl;
    }
  }
}
