#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

const int P = 15;
const int M = 15;
ll midp[P][1 << M];
ll madp[P][1 << M];
ll solve(int m, int plus, int minus, const VL& mas, const VL &mis) {
  if (plus == 0) {
    // divide
    int foreach = m / minus;
    VI bits(minus);
    REP(i, 0, m) {
      bits[i % minus] |= 1 << i;
    }
    ll tot = 0;
    REP(i, 0, minus) {
      tot -= mis[bits[i]];
    }
    return tot;
  }
  //greedy
  int maxlen = m - (plus + minus - 1);
  ll tot = mas[(1 << m) - (1 << (m - maxlen))];
  REP(i, 0, minus) {
    tot -= mis[1 << i];
  }
  REP(i, minus, plus + minus - 1) {
    tot += mis[1 << i];
  }
  return tot;
}

int main(void){
  int n;
  cin >> n;
  int plus = 0, minus = 0;
  VI num;
  REP(i, 0, n) {
    char c;
    cin >> c;
    if (c == '+') {
      plus++;
    } else if (c == '-') {
      minus++;
    } else {
      num.push_back(c - '0');
    }
  }
  sort(num.begin(), num.end());
  plus++;
  ll ma = -1e18, mi = 1e18;
  int m = num.size();
  VL mas(1 << m), mis(1 << m);
  REP(bits, 0, 1 << m) {
    ll a = 0, b = 0;
    ll cur = 1;
    REP(i, 0, m) {
      if (bits & (1 << i)) {
	a *= 10;
	a += num[i];
	b += cur * num[i];
	cur *= 10;
      }
    }
    mas[bits] = b;
    mis[bits] = a;
  }
  ma = solve(m, plus, minus, mas, mis);
  mi = -solve(m, minus, plus, mas, mis);
  cout << ma << " " << mi << endl;
}
