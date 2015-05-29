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


struct sci {
  double m;
  ll e;
  sci operator *(const sci & other) const {
    double r = m * other.m;
    ll re = e + other.e;
    if (r >= 10) {
      r /= 10;
      re++;
    }
    return (sci) {r, re};
  }
};


int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    ll a, b;
    cin >> a >> b;
    double m_a = a;
    ll e_a = 0;
    while (m_a >= 10) {
      m_a /= 10;
      e_a++;
    }
    sci s = (sci) {m_a, e_a};
    sci sum = (sci) {1, 0};
    sci cur = s;
    while (b > 0) {
      if (b % 2) {
	sum = sum * cur;
      }
      cur = cur * cur;
      b /= 2;
    }

    m_a = sum.m;
    cout << (int)m_a << " " << ((int)(m_a * 10) % 10) << " " << sum.e << endl;
  }
}
