#![cfg_attr(not(feature = "std"), no_std)]

// Re-exporta o pallet para ser usado no runtime.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// A trait de configuração do pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A runtime precisa implementar este tipo de evento.
        /// O pallet usa isso para depositar eventos.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Armazenamento (Storage) do pallet.
    /// Aqui é onde você guarda o estado do seu pallet.
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Um item de storage simples para guardar um número.
    pub type Something<T> = StorageValue<_, u32>;

    /// Eventos que o pallet pode emitir.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Evento emitido quando `do_something` é chamado com sucesso.
        /// [valor, quem_chamou]
        SomethingStored { something: u32, who: T::AccountId },
    }

    /// Erros que o pallet pode retornar.
    #[pallet::error]
    pub enum Error<T> {
        /// Um erro de exemplo.
        NoneValue,
    }

    /// As funções que podem ser chamadas de fora do runtime (extrinsics).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Uma função "callable" de exemplo que recebe um valor e o armazena.
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().writes(1) + Weight::from_parts(10_000, 0))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Garante que a chamada veio de uma conta assinada.
            let who = ensure_signed(origin)?;

            // Atualiza o valor no storage.
            <Something<T>>::put(something);

            // Emite um evento para notificar que algo aconteceu.
            Self::deposit_event(Event::SomethingStored { something, who });

            // Retorna Ok para indicar que a execução foi bem-sucedid/as
            Ok(())
        }
    }
}