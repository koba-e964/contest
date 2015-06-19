#include <iostream>
#include <sstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef unsigned long long int ull;
const double EPS=1e-9;

const int N = 40000;

const int W = 64;
ull a[N];
ull room[10000];


int main(void){
  int n,s,x,y,z,q;
  cin >> n >> s >> x >> y >> z >> q;
  ull tmp = s;
  REP(i, 0, n) {
    a[i / W] |= (tmp % 2) << (i % W);
    tmp = tmp * x + y;
    tmp %= z;
  }
  REP(loop_var, 0, q) {
    int s,t,u,v;
    cin >> s >> t >> u >> v;
    s--, u--;
    ull s64 = s % 64;
    if (s / 64 == t / 64) {
      room[0] = a[s/64] >> s64;
      room[1] = 0;
    } else {
      room[0] = a[s/64] >> s64;
      REP(i, s / 64 + 1, t / 64 + 1) {
	room[i - s / 64 - 1] |= s64 ? (a[i] << (64 - s64)) : 0;
	room[i - s / 64] = a[i] >> s64;
      }
      room[t / 64 - s / 64 + 1] = 0;
    }
    ull u64 = u % 64;
    ull v64 = v % 64;
    if (u / 64 == v / 64) {
      a[u / 64] ^= (room[0] << u64) & ((1ULL << v64) - 1);
    } else {
      a[u / 64] ^= room[0] << u64;
      REP(i, u / 64 + 1, v / 64) {
	a[i] ^= room[i - u / 64] << u64;
	a[i] ^= u64 ? room[i - u / 64 - 1] >> (64 - u64) : 0;
      }
      ull sk = room[v / 64 - u / 64] << u64;
      sk ^= u64 ? room[v / 64 - u / 64 - 1] >> (64 - u64) : 0;      
      sk &= (1ULL << v64) - 1;
      a[v / 64] ^= sk;
    }
  }
  stringstream ss;
  REP(i, 0, n) {
    ss << (a[i / 64] & (1ULL << (i % 64)) ? 'O' : 'E');
  }
  cout << ss.str();
  cout << endl;
}
