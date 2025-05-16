// use crate::commitment::Commitment;
use crate::commitment::Commitment;
use crate::public_params::PublicParams;
use crate::schnorr::SchnorrProtocol;
use ark_ec::pairing::Pairing;
use ark_ff::UniformRand;
use ark_std::rand::Rng;

#[derive(Debug, Clone)]
pub struct CommitmentProof<E: Pairing> {
    pub commitment: Commitment<E>,
    pub schnorr_commitment: E::G1Affine,
    pub bases: Vec<E::G1Affine>,
    pub challenge: E::ScalarField,
    pub responses: Vec<E::ScalarField>,
}

impl<E: Pairing> CommitmentProof<E> {
    pub fn prove(
        pp: &PublicParams<E>,
        commitment: &Commitment<E>,
        messages: &[E::ScalarField],
        r: &E::ScalarField,
        rng: &mut impl Rng,
    ) -> Self {
        // Get bases and exponents for the proof
        let bases = pp.get_g1_bases();

        // Create a new vector with copies of messages and add r at the end
        let mut exponents = messages.to_vec();
        exponents.push(*r);

        // Generate Schnorr commitment
        let schnorr_commitment = SchnorrProtocol::commit(&bases, rng);

        // Generate challenge
        let challenge = E::ScalarField::rand(rng);

        // Generate responses - use exponents which includes r, not just messages
        let responses = SchnorrProtocol::prove(&schnorr_commitment, &exponents, &challenge);

        // Create CommitmentProof
        let proof: CommitmentProof<E> = CommitmentProof {
            commitment: commitment.clone(),
            schnorr_commitment: schnorr_commitment.commited_blindings,
            bases,
            challenge,
            responses: responses.0,
        };

        proof
    }

    pub fn verify(&self) -> bool {
        // Verify using Schnorr protocol
        let is_valid = SchnorrProtocol::verify_schnorr(
            &self.bases,
            &self.commitment.cm,
            &self.schnorr_commitment,
            &self.responses,
            &self.challenge,
        );

        is_valid
    }
}
