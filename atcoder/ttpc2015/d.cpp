#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;

bool is_prime(ll n) {
  if (n <= 1) {
    return 0;
  }
  for (ll i = 2; i * i <= n; ++i) {
    if (n % i == 0) {
      return n == i;
    }
  }
  return 1;
}

ll dfs(const vector<ll> &ma, int bits, ll sum) {
  int p = __builtin_popcount(bits);
  int n = ma.size();
  if (p >= n) {
    if (is_prime(sum)) {
      return sum;
    }
    return -1;
  }
  REP(i, 0, 5) {
    if (bits & (1 << i)) {
      continue;
    }
    ll res = dfs(ma, bits | (1 << i), sum + ma[p] * (2 * i + 1));
    if (res != -1) {
      return res;
    }
  }
  return -1;
}

int main(void){
  string s;
  cin >> s;
  set<char> st;
  REP(i, 0, s.length()) {
    st.insert(s[i]);
  }
  if (st.size() >= 6) {
    cout << -1 << endl;
    return 0;
  }
  map<char, ll> ma;
  for (set<char>::iterator it = st.begin();  it != st.end(); ++it) {
    ma[*it] = 0;
  }
  ll c = 1;
  REP(i, 0, s.length()) {
    ma[s[s.length() - 1 - i]] += c;
    c *= 10;
  }
  vector<ll> p;
  for (map<char, ll>::iterator it = ma.begin();  it != ma.end(); ++it) {
    p.push_back(it->second);
  }
  cout << dfs(p, 0, 0) << endl;
}
