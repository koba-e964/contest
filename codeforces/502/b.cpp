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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string a, b;
  cin >> n >> a >> b;
  ll zero = 0, one = 0;
  ll zero_all = 0, one_all = 0;
  REP(i, 0, n) {
    if (a[i] == '0') zero_all++;
    else one_all++;
    if (b[i] == '0') continue;
    if (a[i] == '0') zero++;
    else one++;
  }
  cout << zero_all * one_all - zero * one << endl;
}
