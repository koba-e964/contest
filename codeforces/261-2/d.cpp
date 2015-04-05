#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

/**
 * holds the array (1.. size)
 *
 */
template <class T> class BIT{
private:
	int n;
	T* ary;
public:
	BIT(int n) {
		if(n<=0)
			assert(0);
		while(n != (n & (-n))) {
			n += n & (-n);
		}
		this->n = n;
		ary = new T[n+1];
		for(int i=0; i<=n; i++) {
			ary[i] = 0;
		}
	}
	~BIT(void) {
		delete [] ary;
	}
	/**
	 * gets the sum in [1 .. ind]
	 * @param ind
	 * @return sum
	 */
	T accum(int ind) {
		T sum=0;
		while(ind>0) {
			sum += ary[ind];
			ind &= ind - 1;
		}
		return sum;
	}
	/**
	 * performs data[ind] += val;
	 */
	void add(int ind, T val) {
		if(ind <= 0) {
			assert(0);
		}
		while(ind <= n) {
			ary[ind] += val;
			ind += ind & (-ind);
		}
	}
	T size() {
		return ary[n];
	}
};

void countup(map<int,int> &t, int v) {
	if(t.find(v) != t.end()) {
		t[v]++;
	} else {
		t.insert(pair<int,int>(v, 1));
	}
}

const int DEBUG = 0;

const int N= 1234510;
int n;
int a[N]={0};

int u[N]={0};
int v[N]={0};
int main(void){
	cin>>n;
	REP(i,0,n) cin >> a[i];
	map<int,int> t;
	
	REP(i,0,n) {
		countup(t,a[i]);
		u[i] = t[a[i]];
	}
	t.clear();
	for(int i = n-1; i >= 0; i--) {
		countup(t,a[i]);
		v[i] = t[a[i]];
	}
	if(DEBUG) {
		REP(i,0,n) {
			cout << "u[" << i << "]=" << u[i] << ",v[" << i << "]=" << v[i] << endl;
		}
	}
	BIT<ll> bit(n+1);
	ll sum=0;
	REP(i,0,n) {
		sum += bit.accum(v[i]);
		bit.add(u[i],1);
	}
	cout << (ll)n * (n - 1) / 2 - sum << endl;
}
